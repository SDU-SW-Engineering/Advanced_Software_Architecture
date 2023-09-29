import { Entity, PrimaryGeneratedColumn, Column } from 'typeorm';

@Entity('numbers')
export class NumberEntity {
    @PrimaryGeneratedColumn()
    id!: number;

    @Column()
    value!: number;
}
